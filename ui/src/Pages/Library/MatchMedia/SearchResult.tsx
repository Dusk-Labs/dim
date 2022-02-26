import TruncText from "Helpers/TruncText";
import "./SearchResult.scss";

export const SearchResult = () => {
  const description =
    "In the distant future, mankind has lived quietly and restlessly underground for hundreds of years, subject to earthquakes and cave-ins. Living in one such village are 2 young men: one named Simon who is shy and naïve, and the other named Kamina who believes in the existence of a “surface” world above their heads.";

  return (
    <div className="result-card">
      <div className="left">
        <img
          src="https://image.tmdb.org/t/p/w200/xL6HsYsk5N9PKwk6jFwMNQq3K3M.jpg"
          alt=""
        />
      </div>
      <div className="right">
        <div className="top-row">
          <p>Gurren Lagann</p>

          <div className="meta">
            <p>8.4</p>
            <p>2008</p>
            <p>1h 52m</p>
          </div>
        </div>

        <div className="middle">
          <p>Sci-fi</p>
          <p>Drama</p>
          <p>Anime</p>
        </div>

        <div className="bottom">
          <div className="description">
            <TruncText content={description} max={20} />
          </div>
        </div>
      </div>
    </div>
  );
};

export default SearchResult;
