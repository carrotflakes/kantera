import { connect } from 'react-redux';
import App from 'components/App';
import * as mainProcess from 'modules/mainProcess';

type ReduxState = {
  mainProcess: mainProcess.State
};

export default connect(
  (state: ReduxState) => ({
    ready: !!state.mainProcess.ws
  }), {
    connect: mainProcess.connect,
    disconnect: mainProcess.disconnect,
    init: mainProcess.init,
    send: mainProcess.send
  })(App);
