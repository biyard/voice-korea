import pkg from 'js-sha3';
const { sha3_256 } = pkg;

export function stringToHex(str: string) {
  return sha3_256(str);
  }  