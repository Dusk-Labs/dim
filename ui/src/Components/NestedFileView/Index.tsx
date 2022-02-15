import { useState, useCallback } from "react";
import FolderIcon from "assets/figma_icons/Folder";
import CheckIcon from "assets/Icons/Check";
import { Collapse } from "react-collapse";
import "./Index.scss";

export const FolderView = (props: any) => {
  const { files, noBorder, label, select, unselect } = props;
  const [isOpen, setOpen] = useState(false);
  const depth = props.depth || 0;
  const paddingLeft = depth * 20;

  const toggleFolder = useCallback(() => {
    setOpen(!isOpen);
  }, [isOpen, setOpen]);

  let children = [];

  for (const file of files) {
    let item;

    if (file.type === "file") {
      item = (
        <FileView
          label={file.name}
          object={file}
          depth={depth + 1}
          select={select}
          unselect={unselect}
        />
      );
    } else {
      item = (
        <FolderView
          label={file.name}
          depth={depth + 1}
          files={file.content || []}
          noBorder
          select={select}
          unselect={unselect}
        />
      );
    }

    children.push(item);
  }

  return (
    <div className={`nested-folder ${!noBorder && "with-border"}`}>
      <div
        className={`folder-details ${isOpen && "is-active"}`}
        style={{ paddingLeft: `${paddingLeft}px` }}
        onClick={toggleFolder}
      >
        <FolderIcon />
        <p className="folder-label">{label}</p>
      </div>

      <Collapse isOpened={isOpen}>
        <div className="nested-children">{children}</div>
      </Collapse>
    </div>
  );
};

const FileView = (props: any) => {
  const { label, depth, select, unselect, object } = props;
  const [isActive, setActive] = useState(false);

  const toggleActive = useCallback(() => {
    setActive(!isActive);

    if (select && unselect) {
      if (isActive) {
        select(object);
      } else {
        unselect(object);
      }
    }
  }, [isActive, setActive, select, unselect, object]);

  const paddingLeft = depth ? depth * 20 : 0;

  return (
    <div
      className={`file-view ${isActive && "is-active"}`}
      style={{ paddingLeft: `${paddingLeft}px` }}
      onClick={toggleActive}
    >
      <div className={`select-box ${isActive && "is-active"}`}>
        <CheckIcon />
      </div>
      <p>{label}</p>
    </div>
  );
};

export const NestedFileView = (props: any) => {
  const { files, select, unselect } = props;
  let folders = [];

  for (const item of files) {
    folders.push(
      <FolderView
        label={item.name}
        files={item.content}
        select={select}
        unselect={unselect}
      />
    );
  }

  return (
    <div className="nested-file-view">
      <div className="nested-view-container">{folders}</div>
    </div>
  );
};

export default NestedFileView;
