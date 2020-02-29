import React from 'react';
import styled from 'styled-components';

const Container = styled.div`
background: #333;
padding: 4px;
white-space: pre-wrap;
`;

type Props = {
  logs: string[],
};

export default ({
  logs,
}: Props) => {
  return (
    <Container>
      {
        logs.map((log, i) => <div key={i}>{log}</div>)
      }
    </Container>
  );
};
