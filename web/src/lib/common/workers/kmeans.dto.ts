export enum Command {
  init,
  step,
  info,
  free,
}

export interface CommandDto {
  command: Command;
  init?: [k: number, data: Uint8Array, label: Uint8Array];
  dist?: number;
  info?: any[];
}
