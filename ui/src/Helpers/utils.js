// converts to HH:MM:SS format
export const formatHHMMSS = (secs) => (
  new Date(secs * 1000).toISOString().substr(11, 8)
);

/*
  maintains the ratio when finding
  new width/height of an element
*/
export const calcNewSize = (
  currentWidth,
  currentHeight,
  newWidth = undefined,
  newHeight = undefined
) => {
  const ratio = currentWidth / currentHeight;

  if (newWidth !== undefined) {
    return Math.round(newWidth / ratio);
  }

  if (newWidth !== undefined) {
    return Math.round(newHeight * ratio);
  }
};

const parseHMS = (str) => {
  const p = str.split(":");

  let s = 0;
  let m = 1;

  while (p.length > 0) {
    s += m * parseFloat(p.pop(), 10);
    m *= 60;
  }

  return s;
};

export const parseVtt = (text) => {
  const cues = [];
  const segs = text.split("\n\n").filter(seg => seg);

  if (segs[0] === "WEBVTT") {
    segs.shift();
  }

  for (let raw_seg of segs) {
    const seg = raw_seg.split("\n").filter((x) => x.length !== 0);
    const [sts, ets] = seg[0].split(" --> ").map((x) => parseHMS(x));

    seg.shift();
    try {
      cues.push(new VTTCue(sts, ets, seg.join(" \n")));
    } catch (e) {
      console.warn(`failed to parse a cue raw_seg=${raw_seg} sts=${sts} ets=${ets}`);
    }
  }

  return cues;
};
