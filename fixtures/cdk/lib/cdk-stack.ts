import * as cdk from "aws-cdk-lib";
import { Construct } from "constructs";

import "dotenv/config";
import * as cloudfront from "aws-cdk-lib/aws-cloudfront";
import * as origins from "aws-cdk-lib/aws-cloudfront-origins";
import * as s3 from "aws-cdk-lib/aws-s3";
import * as acm from "aws-cdk-lib/aws-certificatemanager";
import * as route53 from "aws-cdk-lib/aws-route53";
import * as targets from "aws-cdk-lib/aws-route53-targets";
import * as lambda from "aws-cdk-lib/aws-lambda";
import * as apigateway from "aws-cdk-lib/aws-apigateway";
import * as dynamodb from "aws-cdk-lib/aws-dynamodb";
import * as ec2 from "aws-cdk-lib/aws-ec2";
import * as event_targets from "aws-cdk-lib/aws-events-targets";
import * as events from "aws-cdk-lib/aws-events";

export class CdkStack extends cdk.Stack {
  constructor(scope: Construct, id: string, props?: cdk.StackProps) {
    super(scope, id, props);

    let domain = process.env.DOMAIN || "";
    let acmId = process.env.ACM_ID || "";
    let hostedZoneId = process.env.HOSTED_ZONE_ID || "";
    let tableName = process.env.TABLE_NAME || "";
    let codePath = process.env.CODE_PATH || "";
    let indexes = [
      {
        name: "type-index",
        partitionKey: "type",
        sortKey: "created_at",
      },
      {
        name: "gsi1-index",
        partitionKey: "gsi1",
        sortKey: "created_at",
      },
      {
        name: "gsi2-index",
        partitionKey: "gsi2",
        sortKey: "created_at",
      },
      {
        name: "auth-key-index",
        partitionKey: "auth_key",
        sortKey: "created_at",
      },
    ];
    let enableDyanmo = process.env.ENABLE_DYNAMO === "true";
    let enableS3 = process.env.ENABLE_S3 === "true";
    let enableCron = process.env.ENABLE_CRON === "true";

    const certificate = acm.Certificate.fromCertificateArn(
      this,
      "Certificate",
      acmId,
    );

    const func = new lambda.Function(this, "Function", {
      runtime: lambda.Runtime.PROVIDED_AL2023,
      code: lambda.Code.fromAsset(codePath),
      handler: "bootstrap",
      environment: {
        NO_COLOR: "true",
      },
      memorySize: 512,
      timeout: cdk.Duration.seconds(30),
    });

    if (enableDyanmo) {
      const table = new dynamodb.Table(this, "DynamoDB", {
        partitionKey: {
          name: "id",
          type: dynamodb.AttributeType.STRING,
        },
        tableName,
        removalPolicy: cdk.RemovalPolicy.RETAIN,
        billingMode: dynamodb.BillingMode.PAY_PER_REQUEST,
      });

      for (let index of indexes) {
        table.addGlobalSecondaryIndex({
          indexName: index.name,
          partitionKey: {
            name: index.partitionKey,
            type: dynamodb.AttributeType.STRING,
          },
          sortKey: {
            name: index.sortKey,
            type: dynamodb.AttributeType.NUMBER,
          },
          projectionType: dynamodb.ProjectionType.ALL,
        });
      }

      table.grantReadWriteData(func);
    }

    const api = new apigateway.LambdaRestApi(this, "Api", {
      handler: func,
      proxy: true,
    });

    let distributionProps: any = {
      defaultBehavior: {
        origin: new origins.RestApiOrigin(api),
        cachePolicy: cloudfront.CachePolicy.CACHING_DISABLED,
        allowedMethods: cloudfront.AllowedMethods.ALLOW_ALL,
        cachedMethods: cloudfront.CachedMethods.CACHE_GET_HEAD_OPTIONS,
        originRequestPolicy:
          cloudfront.OriginRequestPolicy.ALL_VIEWER_EXCEPT_HOST_HEADER,
      },
      domainNames: [domain],
      certificate,
    };
    if (enableCron) {
      const rule = new events.Rule(this, "VoiceKoreaWatcherRule", {
        schedule: events.Schedule.expression("cron(0 15 * * ? *)"), //KST 00:00
      });

      rule.addTarget(new event_targets.LambdaFunction(func));
    }
    if (enableS3) {
      const assetsBucket = new s3.Bucket(this, "Bucket", {
        bucketName: domain,
        removalPolicy: cdk.RemovalPolicy.DESTROY,
      });

      const s3Origin = new origins.S3Origin(assetsBucket);
      distributionProps.additionalBehaviors = {
        "/assets/*": {
          origin: s3Origin,
          cachePolicy: cloudfront.CachePolicy.CACHING_OPTIMIZED,
        },
        "/*.js": {
          origin: s3Origin,
          cachePolicy: cloudfront.CachePolicy.CACHING_OPTIMIZED,
        },
        "/*.css": {
          origin: s3Origin,
          cachePolicy: cloudfront.CachePolicy.CACHING_OPTIMIZED,
        },
        "/*.html": {
          origin: s3Origin,
          cachePolicy: cloudfront.CachePolicy.CACHING_OPTIMIZED,
        },
        "/*.ico": {
          origin: s3Origin,
          cachePolicy: cloudfront.CachePolicy.CACHING_OPTIMIZED,
        },
        "/*.svg": {
          origin: s3Origin,
          cachePolicy: cloudfront.CachePolicy.CACHING_OPTIMIZED,
        },
        "/*.avif": {
          origin: s3Origin,
          cachePolicy: cloudfront.CachePolicy.CACHING_OPTIMIZED,
        },
        "/*.wasm": {
          origin: s3Origin,
          cachePolicy: cloudfront.CachePolicy.CACHING_OPTIMIZED,
        },
        "/icons/*": {
          origin: s3Origin,
          cachePolicy: cloudfront.CachePolicy.CACHING_OPTIMIZED,
        },
        "/images/*": {
          origin: s3Origin,
          cachePolicy: cloudfront.CachePolicy.CACHING_OPTIMIZED,
        },
      };
    }

    const cf = new cloudfront.Distribution(
      this,
      "Distribution",
      distributionProps,
    );

    const zone = route53.HostedZone.fromHostedZoneAttributes(
      this,
      "zone-attribute",
      {
        zoneName: domain,
        hostedZoneId,
      },
    );

    new route53.ARecord(this, "IpV4Record", {
      zone,
      target: route53.RecordTarget.fromAlias(new targets.CloudFrontTarget(cf)),
    });

    new route53.AaaaRecord(this, "IpV6Record", {
      zone,
      target: route53.RecordTarget.fromAlias(new targets.CloudFrontTarget(cf)),
    });
  }
}
