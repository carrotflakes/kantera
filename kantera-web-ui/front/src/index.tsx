import React from 'react';
import ReactDOM from 'react-dom';
import { createStore, applyMiddleware, compose } from 'redux';
import { Provider } from 'react-redux';
import createSagaMiddleware from 'redux-saga';
import { createGlobalStyle } from 'styled-components';
import 'ress';
import { rootReducer, rootSaga } from 'src/modules';
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

const GlobalStyle = createGlobalStyle`
* {
  color: #555;
}
`;

ReactDOM.render(
  <Provider store={store}>
    <GlobalStyle/>
    <App/>
  </Provider>,
  document.getElementById('app'));
