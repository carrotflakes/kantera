import React from 'react';
import styled from 'styled-components';

const Container = styled.div`
background: #333;
padding: 4px;
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
        logs.map(log => <div>{log.replace(/\n/g, '<br/>')}</div>)
      }
    </Container>
  );
};
