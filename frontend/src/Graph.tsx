import { useParams } from "react-router-dom";
import Viewer from "react-viewer";
import { API_URL } from "./constants";

function Graph() {
  const { graphType } = useParams<{ graphType: string }>();

  return (
    <Viewer
      visible={true}
      images={[
        {
          src: `${API_URL}/graph/${graphType}`,
        },
      ]}
    ></Viewer>
  );
}

export default Graph;
