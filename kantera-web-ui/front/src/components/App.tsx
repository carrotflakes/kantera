import React from 'react';
import styled from 'styled-components';
import MonacoEditor from 'components/MonacoEditor';
import axios from 'axios';
import config from 'src/config';
import LogsView from 'containers/LogsView';

const localStorageCodeKey = 'kantera-web-ui/code';
const initialCode = `(set framerate 20)\n(set transparent (rgba 0.0 0.0 0.0 1.0))\n(set font (import_ttf "./tmp/IPAexfont00401/ipaexg.ttf"))\n(set video\n  (composite\n    (vec (plain (rgb 0.0 1.0 0.0)) 'normal)\n    (vec (image_render (text_to_image "Hello, kantera!" 50.0 font) transparent) 'normal)))\n`;

const Button = styled.button`
background: #333;
margin: 4px;
padding: 2px 4px;

&:hover {
  background: #444;
}
`;

const VerticalBox = styled.div`
display: flex;
flex-direction: row;

& > div {
  flex: 1 0 100px;
  overflow: hidden;
}
`;

type Props = {
  ready: boolean,
  connect: () => void,
  disconnect: () => void,
  init: (imgEl: HTMLElement) => void,
  send: (text: string) => void,
  requestRender: (fileName: string) => void
};

export default ({
  ready,
  connect,
  disconnect,
  init,
  send,
  requestRender
}: Props) => {
  const [code, setCode] = React.useState(localStorage.getItem(localStorageCodeKey) || initialCode);
  const selectFileRef = React.useRef<HTMLInputElement>(null);
  const imgRef = React.useCallback(node => {
    if (node !== null) {
      init(node);
    }
  }, []);
  React.useEffect(() => {
    window.addEventListener('unload', e => {
      localStorage.setItem(localStorageCodeKey, code);
    });
  });

  const apply = (code: string) => {send('script: ' + code);};
  const fileUpload = async (e: React.SyntheticEvent) => {
    e.preventDefault();
    if (selectFileRef.current === null || !selectFileRef.current.files)
      return false;
    const params = new FormData();
    params.append('file', selectFileRef.current.files[0]);
    const res = await axios.post(config.serverUrl + 'upload', params, {headers: {'content-type': 'multipart/form-data'}});
    console.log(res);
    setCode(res.data.map((fp: string) => {
      const ext = fp.slice(fp.lastIndexOf('.') + 1);
      if (['png', 'jpg', 'jpeg', 'gif'].includes(ext))
        return `(set image0 (import_image ${JSON.stringify(fp)}))`;
      if (['mp3', 'wav', 'ogg'].includes(ext))
        return `(set audio0 (import_audio ${JSON.stringify(fp)}))`;
      if (['ttf'].includes(ext))
        return `(set font0 (import_ttf ${JSON.stringify(fp)}))`;
      else
        return `; uploaded: ${JSON.stringify(fp)}`;
    }).join('\n') + '\n' + code);
    return false;
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
        <input type="file" id="uploadFile" ref={selectFileRef} name="file" accept=".png,.jpg,.jpeg,.gif,.mp3,.wav,.ogg,.ttf" style={{display: "none"}}/>
        <Button onClick={fileUpload}>submit</Button>
      </form>
      <Button onClick={() => requestRender('video.mp4')}>rendering</Button>
      <VerticalBox>
        <div>
          <MonacoEditor
            value={code}
            apply={apply}
            onChange={(newValue) => setCode(newValue)}/>
        </div>
        <div>
          <img ref={imgRef}></img>
        </div>
      </VerticalBox>
      <LogsView/>
    </div>
  );
};
