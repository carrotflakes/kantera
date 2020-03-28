import React from 'react';
import { Line } from "react-konva";

export const GridV = ({
  x,
  y,
  width,
  height,
  dx,
  stroke
}: {
  x: number,
  y: number,
  width: number,
  height: number,
  dx: number,
  stroke: string
}) => {
  const n = (width / dx | 0) + 1;
  const lines = Array.from(Array(n).keys()).map(i => (<Line points={[x + i * dx, y, x + i * dx, y + height]} stroke={stroke} strokeWidth={1} key={i}/>));
  return (<>{lines}</>);
};

export const GridH = ({
  x,
  y,
  width,
  height,
  dy,
  stroke
}: {
  x: number,
  y: number,
  width: number,
  height: number,
  dy: number,
  stroke: string
}) => {
  const n = (height / dy | 0) + 1;
  const lines = Array.from(Array(n).keys()).map(i => (<Line points={[x, y + i * dy, x + width, y + i * dy]} stroke={stroke} strokeWidth={1} key={i}/>));
  return (<>{lines}</>);
};
