import { Object3D } from "three";

declare global {
  namespace JSX {
    interface IntrinsicElements {
      group: React.DetailedHTMLProps<
        { ref?: React.RefObject<Object3D> } & any,
        any
      >;
      mesh: React.DetailedHTMLProps<
        { ref?: React.RefObject<Object3D> } & any,
        any
      >;
      lineSegments: React.DetailedHTMLProps<
        { ref?: React.RefObject<Object3D> } & any,
        any
      >;
      octahedronGeometry: React.DetailedHTMLProps<any, any>;
      tetrahedronGeometry: React.DetailedHTMLProps<any, any>;
      boxGeometry: React.DetailedHTMLProps<any, any>;
      icosahedronGeometry: React.DetailedHTMLProps<any, any>;
      dodecahedronGeometry: React.DetailedHTMLProps<any, any>;
      torusGeometry: React.DetailedHTMLProps<any, any>;
      ringGeometry: React.DetailedHTMLProps<any, any>;
      planeGeometry: React.DetailedHTMLProps<any, any>;
      cylinderGeometry: React.DetailedHTMLProps<any, any>;
      coneGeometry: React.DetailedHTMLProps<any, any>;
      capsuleGeometry: React.DetailedHTMLProps<any, any>;
      edgesGeometry: React.DetailedHTMLProps<any, any>;
      lineBasicMaterial: React.DetailedHTMLProps<any, any>;
      meshBasicMaterial: React.DetailedHTMLProps<any, any>;
      ambientLight: React.DetailedHTMLProps<any, any>;
      pointLight: React.DetailedHTMLProps<any, any>;
      color: React.DetailedHTMLProps<any, any>;
    }
  }
}
