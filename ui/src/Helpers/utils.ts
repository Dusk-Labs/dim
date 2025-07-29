// converts to HH:MM:SS format
export const formatHHMMSS = (secs: number) =>
  new Date(secs * 1000).toISOString().substring(11, 19);

// converts to HH:MM:SS + Date format
export const formatHHMMSSDate = (timestamp: number) => {
  const ts = new Date(timestamp * 1000);

  return {
    hours: ("0" + ts.getHours()).slice(-2),
    mins: ("0" + ts.getMinutes()).slice(-2),
    secs: ("0" + ts.getSeconds()).slice(-2),
    date: ("0" + ts.getDate()).slice(-2),
    month: ("0" + ts.getMonth()).slice(-2),
    year: ts.getFullYear(),
  };
};

/*
  maintains the ratio when finding
  new width/height of an element
*/
export const calcNewSize = (
  currentWidth: number,
  currentHeight: number,
  newWidth?: number,
  newHeight?: number
) => {
  const ratio = currentWidth / currentHeight;

  if (newWidth !== undefined) {
    return Math.round(newWidth / ratio);
  }

  if (newHeight !== undefined) {
    return Math.round(newHeight * ratio);
  }
};

const parseHMS = (str: string) => {
  const p = str.split(":");

  let s = 0;
  let m = 1;

  while (p.length > 0) {
    const segment = p.pop();

    if (segment != null) {
      s += m * parseFloat(segment);
      m *= 60;
    }
  }

  return s;
};

export const parseVtt = (text: string) => {
  const cues = [];
  const segs = text.split("\n\n").filter((seg) => seg);

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
      console.warn(
        `failed to parse a cue raw_seg=${raw_seg} sts=${sts} ets=${ets}`
      );
    }
  }

  return cues;
};

export const truncate = (word: string, decimals: number) => {
  const parsed = parseFloat(word);
  return parsed.toFixed(decimals);
};
