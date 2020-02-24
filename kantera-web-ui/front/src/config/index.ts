export default process.env.NODE_ENV === 'production' ? {
  webSocketUrl: location.protocol.replace('http', 'ws') + '//' + location.host + '/ws/'
} : {
  webSocketUrl: 'ws://localhost:8080/ws/'
};
