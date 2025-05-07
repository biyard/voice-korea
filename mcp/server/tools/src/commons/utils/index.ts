import crypto from 'crypto';

export function stringToHex(str: string) {
    return crypto.createHash('sha256').update(str).digest('hex');
  }  