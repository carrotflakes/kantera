import React from 'react';
import MonacoEditor from 'react-monaco-editor';
import monacoEditor from 'monaco-editor';
import config from 'src/config';


type Props = {
  value: string,
  onChange: (code: string) => void,
  apply: (code: string) => void,
  mount: (code: string, position: number) => void
};
export default ({
  value,
  onChange,
  apply,
  mount
}: Props) => {
  const [onResize, setOnResize] = React.useState<(() => void) | null>(null);
  const editorDidMount = (editor: monacoEditor.editor.IStandaloneCodeEditor, monaco: typeof monacoEditor) => {
    const model = editor.getModel();
    model?.updateOptions(config.monacoEditorModelOption);
    editor.setModel(model);
    editor.addAction({
      id: 'apply kantera code',
      label: 'Apply',
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
    editor.addAction({
      id: 'mount',
      label: 'Mount',
      keybindings: [
        monaco.KeyMod.CtrlCmd | monaco.KeyCode.KEY_M
      ],
      precondition: undefined,
      keybindingContext: undefined,
      contextMenuGroupId: 'development', // ?
      contextMenuOrder: 2.0,
      run(editor: monacoEditor.editor.IStandaloneCodeEditor) {
        const position = editor.getPosition();
        const model = editor.getModel();
        if (!position || !model) {
          window.alert('cursor not exists');
          return;
        }
        const offset = model.getOffsetAt(position);
        mount(editor.getValue(), offset);
      }
    });
    const resized = () => editor.layout();
    window.addEventListener('resize', resized);
    setOnResize(resized);
  };
  React.useEffect(() => {
    return () => {onResize && window.removeEventListener('resize', onResize)};
  }, []);
  return (
    <MonacoEditor
      //width="800"
      height="400"
      language="kanteraScript"
      theme="vs-dark"
      value={value}
      onChange={onChange}
      options={{fontSize: 12, minimap: {enabled: false}}}
      editorDidMount={editorDidMount}/>
  )
};
