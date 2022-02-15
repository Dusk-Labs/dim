import NestedFileView from "Components/NestedFileView/Index";
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
          <p className="match-label">View and select files to match.</p>

          <NestedFileView files={files} />
        </div>
        <div className="match-right"></div>
      </div>
    </div>
  );
};

export default MatchMedia;
