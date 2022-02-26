import { useState, useCallback } from "react";

export interface ISearchTag {
  name: string;
  content: string;
}

export const SearchTag = (props: ISearchTag) => {
  const { name, content } = props;

  return (
    <span className="advanced-search-tag">
      <p>
        {name}: {content}
      </p>
    </span>
  );
};

export interface TagHook {
  activeTags: Array<ISearchTag>;

  appendTag: (tag: ISearchTag) => void;
  setTagValue: (name: string, value: string) => void;
  popTag: () => ISearchTag | null;
  getTagValue: (tag: string) => ISearchTag | undefined;
  lastTag: () => ISearchTag | null;
}

export const useSearchTags = (): TagHook => {
  const [activeTags, setActiveTags] = useState<Array<ISearchTag>>([]);

  const appendTag = useCallback(
    (tag) => {
      setActiveTags([...activeTags, tag]);
    },
    [activeTags, setActiveTags]
  );

  const setTagValue = useCallback(
    (tag, value) => {
      const tags = [...activeTags];
      const selectedTag = tags.findIndex((x) => x.name === tag);

      tags.splice(selectedTag, 1);

      setActiveTags([...tags, { name: tag, content: value }]);
    },
    [activeTags, setActiveTags]
  );

  const popTag = useCallback(() => {
    const tags = [...activeTags];
    const last = tags.splice(tags.length - 1, 1);

    setActiveTags(tags);

    return last[0];
  }, [activeTags, setActiveTags]);

  const getTagValue = useCallback(
    (tag: string) => {
      return activeTags.find((x) => x.name === tag);
    },
    [activeTags]
  );

  const lastTag = useCallback(() => {
    if (activeTags.length === 0) return null;

    return activeTags[activeTags.length - 1];
  }, [activeTags]);

  return {
    activeTags,
    appendTag,
    setTagValue,
    popTag,
    getTagValue,
    lastTag,
  };
};
