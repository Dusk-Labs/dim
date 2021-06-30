import React, { useCallback, useEffect } from "react";
import { useParams } from "react-router-dom";
import { useDispatch, useSelector } from "react-redux";

import { fetchLibraryUnmatched } from "../../actions/library.js";
import CardList from "../../Components/CardList/Index.jsx";
import UnmatchedCardList from "../../Components/CardList/UnmatchedCardList.jsx";

const Library = () => {
  const dispatch = useDispatch();

  const { unmatched, ws } = useSelector(store => ({
    unmatched: store.library.fetch_library_unmatched,
    ws: store.ws
  }));

  const params = useParams();

  const handleWS = useCallback((e) => {
    const { type } = JSON.parse(e.data);

    if (type === "EventRemoveCard" || type === "EventNewCard") {
      dispatch(fetchLibraryUnmatched(params.id));
    }
  }, [dispatch, params.id]);

  useEffect(() => {
    if (!ws.conn) return;

    ws.conn.addEventListener("message", handleWS);

    return () => {
      ws.conn.removeEventListener("message", handleWS);
    };
  }, [handleWS, ws]);

  useEffect(() => {
    dispatch(fetchLibraryUnmatched(params.id));
  }, [dispatch, params.id]);

  const { fetched, items } = unmatched;

  return (
    <div className="library">
      <CardList path={`/api/v1/library/${params.id}/media`} actions={true} libId={params.id} />
      {(fetched && Object.keys(items).length > 0) && (
        <>
          <div className="separator"/>
          <UnmatchedCardList cards={unmatched}/>
        </>
      )}
    </div>
  );
};

export default Library;
