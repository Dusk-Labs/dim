import { useParams } from "react-router-dom";

import Cards from "./Cards";

import MatchMedia from "./MatchMedia/Index";
import { useGetUnmatchedMediaFilesQuery } from "api/v1/unmatchedMedia";

import "./Index.scss";

interface LibraryParams {
  id?: string | undefined;
}

const Library = () => {
  const { id } = useParams<LibraryParams>();
  const { data, refetch } = useGetUnmatchedMediaFilesQuery(id!);

  return (
    <div className="library">
      {data && data.count > 0 && <MatchMedia data={data} refetch={refetch} />}
      <Cards />
    </div>
  );
};

export default Library;
