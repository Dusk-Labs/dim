import React, { useEffect } from "react";
import { connect } from "react-redux";

import Banners from "../Components/Banners/Index";
import GhostCards from "../Components/CardList/Ghost";
import CardList from "../Components/CardList/Index";
import NewLibraryModal from "../Modals/NewLibrary/Index";

import "./Dashboard.scss";

function Dashboard(props) {
  useEffect(() => {
    document.title = "Dim - Dashboard";
  }, [])

  return (
    <div className="dashboard">
      {Object.values(props.cards.items).flat().length > 0 && (
        <>
          <Banners/>
          <CardList path={`//${window.host}:8000/api/v1/dashboard`}/>
        </>
      )}
      {Object.values(props.cards.items).flat().length === 0 && (
        <>
          <div className="emptyDashboard">
            <h2>Add your first library</h2>
            <p>
              You will be able to see all the media from your
              libraries here, organized for quick and easy access.
            </p>
            <NewLibraryModal>
              <button>Add library</button>
            </NewLibraryModal>
          </div>
          <div className="card_list">
            <GhostCards/>
          </div>
        </>
      )}
    </div>
  );
};

const mapStateToProps = (state) => ({
  cards: state.card.cards
});

const mapActionsToProps = {};

export default connect(mapStateToProps, mapActionsToProps)(Dashboard);
