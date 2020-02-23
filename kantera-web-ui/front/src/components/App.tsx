import React from 'react';
import styled from 'styled-components';
import MonacoEditor from 'react-monaco-editor';
import monacoEditor from 'monaco-editor';

const Button = styled.button`
background: #eee;
margin: 4px;
padding: 2px 4px;
`;

type Props = {
  ready: boolean,
  connect: any,
  disconnect: any,
  init: any,
  send: any
};

export default ({
  ready,
  connect,
  disconnect,
  init,
  send
}: Props) => {
  const [code, setCode] = React.useState('code!!');
  const imgRef = React.useCallback(node => {
    if (node !== null) {
      init(node);
    }
  }, []);
  const apply = (code: string) => {send('script: ' + code);};
  const editorDidMount = (editor: monacoEditor.editor.IStandaloneCodeEditor, monaco: typeof monacoEditor) => {
    editor.addAction({
      id: 'apply kantera code',
      label: 'apply',
      keybindings: [
        monaco.KeyMod.CtrlCmd | monaco.KeyCode.Enter
      ],
      precondition: undefined,
      keybindingContext: undefined,
      contextMenuGroupId: 'development', // ?
      contextMenuOrder: 2.0,
      run(editor: monacoEditor.editor.IStandaloneCodeEditor) {
        apply(editor.getValue());
      }
    });
  };
  return (
    <div>
      {
        ready ?
        <Button onClick={disconnect}>disconnect</Button> :
        <Button onClick={connect}>connect</Button>
      }
      <Button onClick={() => apply(code)}>apply</Button>
      <form style={{display: 'inline-block'}}>
        <label htmlFor="uploadFile">
          select file
        </label>
        <input type="file" id="uploadFile" name="file" accept=".png,.jpg,.jpeg,.gif,.mp3,.wav,.ogg,.ttf" style={{display: "none"}}/>
        <Button id="uploadButton">submit</Button>
      </form>
      <MonacoEditor
        width="800"
        height="400"
        language="scheme"
        theme="vs-dark"
        value={code}
        onChange={(newValue) => setCode(newValue)}
        options={{fontSize: 12, minimap: {enabled: false}}}
        editorDidMount={editorDidMount}/>
      <img ref={imgRef}></img>
    </div>
  );
};