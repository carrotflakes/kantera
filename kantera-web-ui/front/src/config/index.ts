export default process.env.NODE_ENV === 'production' ? {
  serverUrl: location.protocol + '//' + location.host + '/',
  webSocketUrl: location.protocol.replace('http', 'ws') + '//' + location.host + '/ws/'
} : {
  serverUrl: 'http://localhost:8080/',
  webSocketUrl: 'ws://localhost:8080/ws/'
};
