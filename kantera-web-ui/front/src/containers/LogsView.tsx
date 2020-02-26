import { connect } from 'react-redux';
import LogsView from 'components/LogsView';
import * as mainProcess from 'modules/mainProcess';

type ReduxState = {
  mainProcess: mainProcess.State
};

export default connect(
  (state: ReduxState) => ({
    logs: state.mainProcess.logs
  }), {
  })(LogsView);
