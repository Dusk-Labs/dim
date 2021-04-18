import React, { useEffect } from "react";
import { useParams } from "react-router-dom";
import { connect } from "react-redux";

import { fetchLibraryUnmatched } from "../../actions/library.js";
import Actions from "./Actions";
import CardList from "../../Components/CardList/Index";
import UnmatchedCardList from "../../Components/CardList/UnmatchedCardList.jsx";

const Library = (props) => {
  const params = useParams();

  const { fetchLibraryUnmatched, auth } = props;
  const { token } = auth;

  useEffect(() => {
    fetchLibraryUnmatched(token, params.id);
  }, [fetchLibraryUnmatched, params.id, token]);

  const { fetched, items } = props.unmatched;

  return (
    <div className="library">
      <CardList path={`//${window.host}:8000/api/v1/library/${params.id}/media`}/>
      {(fetched && Object.keys(items).length > 0) && (
        <>
          <div className="separator"/>
          <UnmatchedCardList cards={props.unmatched}/>
        </>
      )}
      <Actions id={useParams().id}/>
    </div>
  );
}

const mapStateToProps = (state) => ({
  auth: state.auth,
  unmatched: state.library.fetch_library_unmatched
});

const mapActionsToProps = {
  fetchLibraryUnmatched
};

export default connect(mapStateToProps, mapActionsToProps)(Library);
