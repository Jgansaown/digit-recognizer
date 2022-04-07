import GzWorker from "./workers/gz.worker?worker";
import KMeansWorker from "./workers/kmeans.worker?worker";
import { Command, CommandDto } from "./workers/kmeans.dto";
import type { MnistDataset } from "./mnist.dataset";

const GZ_WORKER = new GzWorker();

export function decode_mnist_gz(files: MnistDataset): Promise<MnistDataset> {
  return new Promise((res, rej) => {
    GZ_WORKER.postMessage(files);
    GZ_WORKER.onmessage = (msg) => res(msg.data);
  });
}

export class Kmeans {
  private worker: Worker;
  private cbs: Record<number, Array<(value?: any) => any>> = {};

  constructor() {
    this.worker = new KMeansWorker();
    this.worker.onmessage = (msg) => this.onmessage(msg);
  }

  private async onmessage(msg: MessageEvent<CommandDto>) {
    switch (msg.data.command) {
      case Command.init:
      case Command.free:
        this.cbs[msg.data.command].forEach((f) => f());
        break;
      case Command.step:
        this.cbs[msg.data.command].forEach((f) => f(msg.data.dist));
        break;
      case Command.info:
        this.cbs[msg.data.command].forEach((f) => f(msg.data.info));
      case Command.test_one:
        this.cbs[msg.data.command].forEach((f) => f(msg.data.label));
      default:
        break;
    }
  }

  private async send_message(msg: CommandDto): Promise<any> {
    return new Promise((res, rej) => {
      this.worker.postMessage(msg);
      if (this.cbs[msg.command] == undefined) {
        this.cbs[msg.command] = [];
      }
      this.cbs[msg.command].push(res);
    });
  }

  async init(k: number, data: Uint8Array, label: Uint8Array) {
    return await this.send_message({
      command: Command.init,
      init: [k, data, label],
    });
  }

  async step(): Promise<number> {
    return await this.send_message({
      command: Command.step,
    });
  }

  async info(): Promise<any> {
    return await this.send_message({
      command: Command.info,
    });
  }

  async free(): Promise<void> {
    return await this.send_message({
      command: Command.free,
    });
  }

  async test_one_rgba(rgba: Uint8Array): Promise<number> {
    return await this.send_message({
      command: Command.test_one,
      image: rgba,
    })
  }
}
