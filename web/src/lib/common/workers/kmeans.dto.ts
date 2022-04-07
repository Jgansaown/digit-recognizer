export enum Command {
  init,
  step,
  info,
  free,
  test_one,
}

export interface CommandDto {
  command: Command;
  init?: [k: number, data: Uint8Array, label: Uint8Array];
  dist?: number;
  info?: any[];
  image?: Uint8Array;
  label?: number;
}
