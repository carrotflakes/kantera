type Buffer = Uint16Array;

export default class AudioManager {
  buffers: Buffer[];
  i: number;
  remoteSampleRate: number;
  channelNum: number;

  constructor() {
    this.buffers = [];
    this.i = 0;
    this.remoteSampleRate = 4000;
    this.channelNum = 2;

    const ctx = new AudioContext();
    console.log('sampleRate: ' + ctx.sampleRate);
    const bufferSize = 4096;
    const scriptNode = ctx.createScriptProcessor(bufferSize, 1, 2);
    scriptNode.onaudioprocess = e => {
      const buffer = e.outputBuffer;
      const array0 = buffer.getChannelData(0);
      const array1 = buffer.getChannelData(1);
      const sampleRate = ctx.sampleRate;
      const remoteSampleRate = this.remoteSampleRate;

      if (this.channelNum === 0) {
        for (let i = 0; i < bufferSize; ++i) {
          array1[i] = array0[i] = 0;
        }
      } else if (this.channelNum === 1) {
        for (let i = 0; i < bufferSize; ++i) {
          if (this.buffers.length) {
            const bufLen = this.buffers[0].length;
            const j = this.i++ * remoteSampleRate / sampleRate | 0;
            array1[i] = array0[i] = this.buffers[0][j] / 2**15 - 1.0;
            if (bufLen <= (this.i * remoteSampleRate / sampleRate | 0)) {
              this.buffers.shift();
              this.i = 0;
            }
          } else {
            array1[i] = array0[i] = 0;
          }
        }
      } else {
        for (let i = 0; i < bufferSize; ++i) {
          if (this.buffers.length) {
            const bufLen = this.buffers[0].length / 2;
            const j = this.i++ * remoteSampleRate / sampleRate | 0;
            array0[i] = this.buffers[0][j] / 2**15 - 1.0;
            array1[i] = this.buffers[0][bufLen + j] / 2**15 - 1.0;
            if (bufLen <= (this.i * remoteSampleRate / sampleRate | 0)) {
              this.buffers.shift();
              this.i = 0;
            }
          } else {
            array1[i] = array0[i] = 0;
          }
        }
      }
    };
    scriptNode.connect(ctx.destination);
  }

  setSamplerate(sampleRate: number) {
    this.remoteSampleRate = sampleRate;
  }

  setChannelNum(channelNum: number) {
    this.channelNum = channelNum;
  }

  push(buffer: Buffer) {
    this.buffers.push(buffer);
    // tekido ni kaihou
    while (this.buffers.length > 4)
      this.buffers.shift();
  }
}
