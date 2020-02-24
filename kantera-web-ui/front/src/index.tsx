import React from 'react';
import ReactDOM from 'react-dom';
import { createStore, applyMiddleware, compose } from 'redux';
import { Provider } from 'react-redux';
import createSagaMiddleware from 'redux-saga';
import 'ress';
import './style.css';
import rootReducer from 'src/modules/reducer';
import rootSaga from 'src/modules/saga';
import App from 'containers/App';
import './monaco';

const composeEnhancers =
  process.env.NODE_ENV === 'development' &&
  typeof window === 'object' &&
  (window as any).__REDUX_DEVTOOLS_EXTENSION_COMPOSE__
    ? (window as any).__REDUX_DEVTOOLS_EXTENSION_COMPOSE__({})
    : compose;

const sagaMiddleWare = createSagaMiddleware();
const enhancer = composeEnhancers(
  applyMiddleware(sagaMiddleWare),
);
export const store = createStore(rootReducer, enhancer);
sagaMiddleWare.run(rootSaga);

ReactDOM.render(
  <Provider store={store}>
    <App/>
  </Provider>,
  document.getElementById('app'));
