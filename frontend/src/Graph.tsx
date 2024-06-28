import { useNavigate, useParams } from "react-router-dom";
import Viewer from "react-viewer";
import { API_URL } from "./constants";

function Graph() {
  const navigate = useNavigate();
  const { graphType } = useParams<{ graphType: string }>();

  return (
    <Viewer
      visible={true}
      onClose={() => navigate("/")}
      noFooter={true}
      images={[
        {
          src: `${API_URL}/graph/${graphType}`,
        },
      ]}
    ></Viewer>
  );
}

export default Graph;
