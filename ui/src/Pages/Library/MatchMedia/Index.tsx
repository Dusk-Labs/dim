import NestedFileView from "Components/NestedFileView/Index";
import SimpleSearch from "Components/SimpleSearch";
import AdvancedSearch from "Components/AdvancedSearch/Index";
import SearchResult from "./SearchResult";

import AngleUp from "assets/Icons/AngleUp";
import "./Index.scss";

const MatchMedia = () => {
  const files = [
    {
      name: "Folder A",
      type: "folder",
      content: [
        {
          name: "Inner Folder A",
          type: "folder",
          content: [
            {
              name: "Folder B",
              type: "folder",
              content: [
                {
                  name: "inner file b",
                  type: "file",
                },
              ],
            },
            {
              name: "File A",
              type: "file",
            },
          ],
        },
      ],
    },
    {
      name: "FOLDER B",
      type: "folder",
      content: [
        {
          name: "inner folder",
          type: "folder",
        },
      ],
    },
    {
      name: "FOLDER B",
      type: "folder",
      content: [
        {
          name: "inner folder",
          type: "folder",
        },
      ],
    },
    {
      name: "FOLDER B",
      type: "folder",
      content: [
        {
          name: "inner folder",
          type: "folder",
        },
      ],
    },
    {
      name: "FOLDER B",
      type: "folder",
      content: [
        {
          name: "inner folder",
          type: "folder",
        },
      ],
    },
    {
      name: "FOLDER B",
      type: "folder",
      content: [
        {
          name: "inner folder",
          type: "folder",
        },
      ],
    },
    {
      name: "FOLDER B",
      type: "folder",
      content: [
        {
          name: "inner folder",
          type: "folder",
        },
      ],
    },
    {
      name: "FOLDER B",
      type: "folder",
      content: [
        {
          name: "inner folder",
          type: "folder",
        },
      ],
    },
    {
      name: "FOLDER B",
      type: "folder",
      content: [
        {
          name: "inner folder",
          type: "folder",
        },
      ],
    },
    {
      name: "FOLDER B",
      type: "folder",
      content: [
        {
          name: "inner folder",
          type: "folder",
        },
      ],
    },
  ];

  return (
    <div className="match-media">
      <div className="match-container">
        <div className="match-left">
          <p className="match-head">3 Unmatched files found</p>
          <div className="match-middle">
            <p className="match-label">View and select files to match.</p>
            <SimpleSearch />
          </div>

          <NestedFileView files={files} />
        </div>
        <div className="match-right">
          <div className="search-head">
            <AdvancedSearch />
            <div className="toggle">
              <AngleUp />
            </div>
          </div>

          <div className="search-results">
            <SearchResult />
            <SearchResult />
            <SearchResult />
          </div>
        </div>
      </div>
    </div>
  );
};

export default MatchMedia;
