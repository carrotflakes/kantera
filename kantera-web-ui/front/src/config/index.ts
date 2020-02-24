const common = {
  monacoEditorModelOption: {tabSize: 2}
};

export default process.env.NODE_ENV === 'production' ? {
  ...common,
  serverUrl: location.protocol + '//' + location.host + '/',
  webSocketUrl: location.protocol.replace('http', 'ws') + '//' + location.host + '/ws/'
} : {
  ...common,
  serverUrl: 'http://localhost:8080/',
  webSocketUrl: 'ws://localhost:8080/ws/'
};
