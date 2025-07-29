import { useCallback, useEffect, useState } from "react";

import { useGetBannersQuery } from "../../../api/v1/dashboard";
import useWebSocket from "../../../hooks/ws";
import Banner from "./Banner";
import Crumbs from "./Crumbs";

import "./Index.scss";

function Banners() {
  const { data: items, error, isFetching, refetch } = useGetBannersQuery();

  const ws = useWebSocket();

  const [activeIndex, setActiveIndex] = useState(0);
  const [currentTimeoutID, setCurrentTimeoutID] = useState<number | null>(null);
  const [throttleEventNewCardID, setThrottleEventNewCardID] = useState<
    number | null
  >(null);

  const handleWS = useCallback(
    (e) => {
      const { type } = JSON.parse(e.data);

      if (items && items.length >= 3) return;

      if (type === "EventNewCard") {
        if (throttleEventNewCardID) {
          clearTimeout(throttleEventNewCardID);
          setThrottleEventNewCardID(null);
        }

        const id = window.setTimeout(() => {
          refetch();
        }, 500);

        setThrottleEventNewCardID(id);
      }
    },
    [items, refetch, throttleEventNewCardID]
  );

  useEffect(() => {
    const timeout = window.setTimeout((timeoutID: number) => {
      if (items && items.length > 0) {
        const nextIndex = activeIndex < items.length - 1 ? activeIndex + 1 : 0;

        setActiveIndex(nextIndex);
        setCurrentTimeoutID(timeoutID);
      } else {
        clearTimeout(timeoutID);
      }
    }, 14000);

    return () => clearTimeout(timeout);
  }, [activeIndex, items]);

  const toggle = useCallback(
    (e) => {
      if (currentTimeoutID) {
        clearTimeout(currentTimeoutID);
      }
      setActiveIndex(parseInt(e.target.dataset.key));
    },
    [currentTimeoutID]
  );

  useEffect(() => {
    if (!ws) return;

    ws.addEventListener("message", handleWS);
    return () => ws.removeEventListener("message", handleWS);
  }, [handleWS, ws]);

  return (
    <div className="banner-wrapper">
      <Banner
        data={items && items[activeIndex]}
        isFetching={isFetching}
        isError={typeof error !== "undefined"}
      />
      {items && items.length > 1 && (
        <Crumbs
          count={items.length}
          toggle={toggle}
          activeIndex={activeIndex}
        />
      )}
    </div>
  );
}

export default Banners;
