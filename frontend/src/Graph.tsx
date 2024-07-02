import { useNavigate, useParams } from "react-router-dom";
import Viewer from "react-viewer";
import { API_URL } from "./constants";
import { BrowserView, MobileView } from "react-device-detect";

function Graph() {
  const navigate = useNavigate();
  const { graphType } = useParams<{ graphType: string }>();

  const imgSrc = `${API_URL}/graph/${graphType}`;

  return (
    <>
      <BrowserView>
        <Viewer
          visible={true}
          onClose={() => navigate("/")}
          noFooter={true}
          zoomSpeed={0.1}
          minScale={0.5}
          images={[
            {
              src: imgSrc,
            },
          ]}
        ></Viewer>
      </BrowserView>
      <MobileView>
        <img src={imgSrc} />
      </MobileView>
    </>
  );
}

export default Graph;
