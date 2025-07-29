import "./ProgressBar.scss";
import CircleIcon from "assets/Icons/Circle";

interface ProgressBarData {
  season: number;
  episode: number;
  delta: number;
  duration: number;
}

interface Props {
  data: ProgressBarData;
}

function ProgressBar(props: Props) {
  const { delta, season, episode } = props.data;
  let { duration } = props.data;

  duration = Math.round(duration / 60);

  const current = Math.round(delta / 60);
  const width = (current / duration) * 100 + "%";

  return (
    <div className="bannerProgressBar">
      {(season || season === 0) && (episode || episode === 0) && (
        <div className="s-e">
          <p>S{season}</p>
          <CircleIcon />
          <p>E{episode}</p>
        </div>
      )}
      <div className="progress">
        <div className="current">
          <p>{current}</p>
          <p>min</p>
        </div>
        <div className="bar">
          <span className="progress-fill" style={{ width: width }} />
        </div>
        <div className="duration">
          <p>{duration}</p>
          <p>min</p>
        </div>
      </div>
    </div>
  );
}

export default ProgressBar;
