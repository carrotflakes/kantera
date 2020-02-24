import { delay, fork, call, take, put, select, takeLatest } from 'redux-saga/effects';
import { eventChannel, END } from 'redux-saga';
import AudioManager from 'src/audioManager';

export const CONNECT = 'MAIN_PROCESS/CONNECT' as const;
export const DISCONNECT = 'MAIN_PROCESS/DISCONNECT' as const;
export const CONNECTED = 'MAIN_PROCESS/CONNECTED' as const;
export const DISCONNECTED = 'MAIN_PROCESS/DISCONNECTED' as const;
export const SEND = 'MAIN_PROCESS/SEND' as const;
export const INIT = 'MAIN_PROCESS/INIT' as const;
export const SET_AUDIO_MANAGER = 'MAIN_PROCESS/SET_AUDIO_MANAGER' as const;

export interface State {
  ws: WebSocket | null,
  imgEl: HTMLElement | null,
  audioManager: AudioManager | null
}

const initialState: State = {
  ws: null,
  imgEl: null,
  audioManager: null
};


type Action
  = ReturnType<typeof connected>
  | ReturnType<typeof disconnected>
  | ReturnType<typeof send>
  | ReturnType<typeof init>
  | ReturnType<typeof setAudioManager>;

const reducers: { [key: string]: (state: State, action: any) => State } = {};

export const connect = () => {
  return {
    type: CONNECT
  };
}

export const disconnect = () => {
  return {
    type: DISCONNECT
  };
}

const connected = (ws: WebSocket) => {
  return {
    type: CONNECTED,
    ws
  };
}
reducers[CONNECTED] = (state: State, action: ReturnType<typeof connected>): State => {
  return {
    ...state,
    ws: action.ws
  };
};

const disconnected = () => {
  return {
    type: DISCONNECTED
  };
}
reducers[DISCONNECTED] = (state: State, action: ReturnType<typeof disconnected>): State => {
  return {
    ...state,
    ws: null
  };
};

export const send = (text: string) => {
  return {
    type: SEND,
    text
  };
}

export const init = (imgEl: HTMLElement) => {
  return {
    type: INIT,
    imgEl
  };
}
reducers[INIT] = (state: State, action: ReturnType<typeof init>): State => {
  return {
    ...state,
    imgEl: action.imgEl
  };
};

export const setAudioManager = (audioManager: AudioManager) => {
  return {
    type: SET_AUDIO_MANAGER,
    audioManager
  };
}
reducers[SET_AUDIO_MANAGER] = (state: State, action: ReturnType<typeof setAudioManager>): State => {
  return {
    ...state,
    audioManager: action.audioManager
  };
};

export function reducer(state: State = initialState, action: Action): State {
  const reducer = reducers[action.type] || ((state, _action) => state) as (state: State, action: Action) => State;
  return reducer(state, action);
}

function connectWebsocket(url: string) : Promise<WebSocket> {
  return new Promise((resolve, reject) => {
    const socket = new WebSocket(url);
    socket.onopen = () => {
      resolve(socket);
    };
    socket.onerror = evt => {
      reject(evt);
    }
  });
}

function* handleConnect(action: ReturnType<typeof connect>) {
  let audioManager = yield select(state => state.mainProcess.audioManager);
  if (audioManager === null) {
    audioManager = new AudioManager();
    yield put(setAudioManager(audioManager));
  }
  const imgEl = yield select(state => state.mainProcess.imgEl);
  const wsUrl = location.protocol.replace('http', 'ws') + '//' + location.host + '/ws/';
  const ws: WebSocket = yield call(connectWebsocket, wsUrl);
  const socketChannel = eventChannel(emit => {
    ws.onmessage = event => emit(event.data);
    ws.onclose = () => emit(END);
    return () => {
      ws.onmessage = null;
    };
  });
  yield put(connected(ws));
  let binaryType: string | null = null;
  let streamInfo = null;
  try {
    while (true) {
      const message = yield take(socketChannel);
      if (message instanceof Blob) {
        if (binaryType === 'frame') {
          imgEl.src = (window.URL || window.webkitURL).createObjectURL(message);
          //history.push(Date.now());
          //if (history.length > 30) history.shift();
        } else if (binaryType === 'audio') {
          const fileReader = new FileReader();
          fileReader.onloadend = () => {
            const array = new Uint16Array(fileReader.result as ArrayBuffer);
            audioManager.push(array);
          };
          fileReader.readAsArrayBuffer(message);
        }
        binaryType = null;
      } else if (typeof message === 'string') {
        const data = eval('(' + message + ')'); // for parse "\'"
        if (data.type === 'sync') {
          //syncObj = {...syncObj, ...data, type: undefined};
        } else if (data.type === 'parseFailed') {
          //document.getElementById('parseError').textContent = data.error;
        } else if (data.type === 'frame') {
          binaryType = 'frame';
        } else if (data.type === 'audio') {
          binaryType = 'audio';
        } else if (data.type === 'streamInfo') {
          streamInfo = data;
          if (streamInfo.samplerate) {
            audioManager.setSamplerate(streamInfo.samplerate);
          }
        } else if (data.type === 'log') {
          //logEl.innerHTML += (data.log + '\n').replace(/\n/, '</br>');
        }
        const mes = JSON.parse(message);
      } else {
        console.log(message);
      }
    }
  } finally {
    yield put(disconnected());
  }
}

function* watchConnect() {
  yield takeLatest(CONNECT, handleConnect);
}

function* handleDisconnect(action: ReturnType<typeof connect>) {
  const ws = yield select(state => state.mainProcess.ws);
  if (ws)
    ws.close();
}

function* watchDisconnect() {
  yield takeLatest(DISCONNECT, handleDisconnect);
}

function* watchSend() {
  while (true) {
    const sendAct: ReturnType<typeof send> = yield take(SEND);
    const ws = yield select(state => state.mainProcess.ws);
    if (ws) {
      ws.send(sendAct.text);
    }
  }
}

export function* saga() {
  yield fork(watchConnect);
  yield fork(watchDisconnect);
  yield fork(watchSend);
}
