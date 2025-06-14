import "./layout.css";

function Layout(props) {
  return (
    <div class="layout">
      <main>{props.children}</main>
    </div>
  );
}

export default Layout;
