import type { Action, ActionReturn } from "svelte/action";

interface PanZoomParams {
  scale?: number;
  panX?: number;
  panY?: number;
}

interface PanZoomAttributes {
  'onzoom'?: (e: CustomEvent<number>) => void;
  'onpanX'?: (e: CustomEvent<number>) => void;
  'onpanY'?: (e: CustomEvent<number>) => void;
}

const panZoom: Action<HTMLDivElement, PanZoomParams, PanZoomAttributes> = (node, parameters) => {
  let { scale = 1, panX = 0, panY = 0 } = parameters;
  const MIN_SCALE = 0.125;
  const MAX_SCALE = 4;

  const onWheel = (event: WheelEvent) => {
    scale += event.deltaY * -0.01;
    scale = Math.min(Math.max(MIN_SCALE, scale), MAX_SCALE);

    node.dispatchEvent(new CustomEvent('zoom', { detail: scale }));
    node.style.transform = `translate(${panX}px, ${panY}px) scale(${scale})`;
  };

  const onDblClick = (_: MouseEvent) => {
    scale = 1;
    panX = 0;
    panY = 0;
    node.style.transform = `translate(${panX}px, ${panY}px) scale(${scale})`;
  }

  const onMouseDown = (event: MouseEvent) => {
    const startX = event.clientX - panX;
    const startY = event.clientY - panY;
    const onMouseMove = (event: MouseEvent) => {
      panX = event.clientX - startX;
      panY = event.clientY - startY;
      node.dispatchEvent(new CustomEvent('panX', { detail: panX }));
      node.dispatchEvent(new CustomEvent('panY', { detail: panY }));
      node.style.transform = `translate(${panX}px, ${panY}px) scale(${scale})`;
    };
    document.addEventListener('mousemove', onMouseMove);
    document.addEventListener('mouseup', () => document.removeEventListener('mousemove', onMouseMove), { once: true });
  };

  if (node.parentElement === null) throw new Error('Parent element not found');
  node.parentElement.addEventListener('wheel', onWheel);
  node.parentElement.addEventListener('mousedown', onMouseDown);
  node.parentElement.addEventListener('dblclick', onDblClick);

  return {
    destroy() {
      if (node.parentElement === null) throw new Error('Parent element not found');
      node.parentElement.removeEventListener('wheel', onWheel);
      node.parentElement.removeEventListener('mousedown', onMouseDown);
    },
    update(value) {
      panX = value.panX ?? panX;
      panY = value.panY ?? panY;
      scale = value.scale ?? scale;
      node.style.transform = `translate(${panX}px, ${panY}px) scale(${scale})`;
    }
  };
}

export { panZoom }
export type { PanZoomParams, PanZoomAttributes }
