import { combineReducers } from 'redux';
import { fork } from 'redux-saga/effects';
import * as mainProcess from './mainProcess';

export const rootReducer = combineReducers({
  mainProcess: mainProcess.reducer
});

export function* rootSaga() {
  yield fork(mainProcess.saga);
}
