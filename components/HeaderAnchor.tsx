"use client";

import { FC, PropsWithChildren } from "react";

interface HeaderAnchorProps {
  id: string;
}

const HeaderAnchor: FC<PropsWithChildren<HeaderAnchorProps>> = ({
  id,
  children,
}) => {
  const handleClick = () => {
    const url = `${window.location.href.split("#")[0]}#${id}`;
    navigator.clipboard.writeText(url);
  };

  return (
    <a href={`#${id}`} className="anchor" onClick={handleClick}>
      {children}
    </a>
  );
};

export default HeaderAnchor;
