import React from 'react';
import styled from 'styled-components';
import Konva from 'konva';
import { Stage, Layer, Rect, Text, Group, Line } from 'react-konva';
import { GridV, GridH } from 'src/konva/Grid';

const ColoredRect = ({
  x,
  z,
  duration,
  caption
}: {
  x: number,
  z: number,
  duration: number | null,
  caption: string
}) => {
  const [loc, setLoc] = React.useState({x, y: z * 50});
  const [ghostLoc, setGhostLoc] = React.useState({x, y: z * 50});
  const locFix = ({x, y}: typeof loc) => ({x: x, y: Math.round(y / 50) * 50});
  const width = (duration === null ? 100 : duration) * 50;
  return (
    <>
      <Group
        x={ghostLoc.x}
        y={ghostLoc.y}>
        <Rect
          width={width}
          height={50}
          fill="#999"
          opacity={0.3}
        />
        <Text
          x={2}
          y={2}
          text={caption}
        />
      </Group>
      <Group
        x={loc.x}
        y={loc.y}
        draggable
        onClick={e => {
        }}
        onDragStart={e => {
          e.target.moveToTop();
          e.target.setAttrs({
            opacity: 0.8
          });
        }}
        onDragMove={e => {
          const loc = locFix({x: e.target.x(), y: e.target.y()});
          setGhostLoc(loc);
        }}
        onDragEnd={e => {
          const loc = locFix({x: e.target.x(), y: e.target.y()});
          e.target.to({
            x: loc.x,
            y: loc.y,
            duration: 0.1,
            opacity: 1.0
          });
          setGhostLoc(loc);
          setLoc(loc);
        }}>
        <Rect
          x={1}
          y={1}
          width={width - 2}
          height={50 - 2}
          fill="#999"
          shadowBlur={5}
        />
        <Text
          x={2}
          y={2}
          text={caption}
        />
      </Group>
    </>
  );
}

export default ({
  data,
  update
}: {
  data: {x: number, z: number, duration: number | null, caption: string}[],
  update: (data: {x: number, z: number, caption: string}[]) => void
}) => {
  const [width, setWidth] = React.useState(600);
  const stageRef = React.useRef<any>(null); // TODO: remove any
  React.useEffect(() => {
    const onResize = () => {
      const konvaContainer = stageRef.current;
      if (konvaContainer instanceof Konva.Container) {
        setWidth(konvaContainer.attrs.container.clientWidth);
      }
    };
    onResize();
    window.addEventListener('resize', onResize);
    return () => {
      window.removeEventListener('resize', onResize);
    };
  });
  const height = 200;
  return (
    <Stage width={width} height={height} style={{width: '100%'}} ref={stageRef}>
      <Layer>
        <Rect x={0} y={0} width={width} height={height} fill="#333"/>
        <GridH x={0} y={0} width={width} height={height} dy={50} stroke="#222"/>
        <GridV x={0} y={0} width={width} height={height} dx={50} stroke="#222"/>
        {
          data.map((c, i) => (<ColoredRect x={c.x * 50} z={c.z} duration={c.duration} caption={c.caption} key={i} />))
        }
        <Rect x={0} y={0} width={width} height={height} stroke="#111" hitFunc={()=> false}/>
      </Layer>
    </Stage>
  );
};
