import { fork } from 'redux-saga/effects';
import { saga as mainProcess } from './mainProcess';

export default function* rootSaga() {
  yield fork(mainProcess);
}
