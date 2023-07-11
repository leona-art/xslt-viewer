import { For, Match, Show, Switch, createMemo, createSignal } from "solid-js";
import FileSelector from "./components/file-selector";
import { xslt } from "./libs/xslt";
import Viewer from "./components/viewer";

const [xml, setXml] = createSignal("")
const [xsl, setXsl] = createSignal("")
const [css, setCss] = createSignal("")


function App() {


  return (
    <>
      <div>
        <div class="dialogs">
          <FileSelector title='XML' ext='xml' setContent={setXml} />
          <FileSelector title='XSL' ext='xsl' setContent={setXsl} />
          <FileSelector title='CSS' ext='css' setContent={setCss} />
        </div>
        <Main />
      </div>
    </>
  );
}

export default App;


function Main() {


  const viewerTypes = ["web", "text", "xml", "xsl", "css"] as const
  type ViewerType = typeof viewerTypes[number];
  const [selectedViewer, setSelectedViewer] = createSignal<ViewerType>("web")

  const result = createMemo(() => xslt(xml(), xsl()))

  // xsltの結果をダウンロードする
  function downloadHandler() {
    const blob = new Blob([result() ?? ""], { type: "text/plain" })
    const a = document.createElement("a")
    a.href = URL.createObjectURL(blob)
    a.download = "result.html"
    a.click()
  };

  return (
    <>
      <div>
        <hr />
        <select value={selectedViewer()} onChange={e => setSelectedViewer(e.currentTarget.value as ViewerType)}>
          <For each={viewerTypes}>
            {item => <option value={item}>{item}</option>}
          </For>
        </select>
        <hr />
        <Show when={result()}>
          {item => (
            <Switch>
              <Match when={selectedViewer() === "web"}>
                <Viewer html={item()} css={css()} />
                <button onClick={downloadHandler}>download</button>
              </Match>
              <Match when={selectedViewer() === "text"}>
                <pre>{item()}</pre>
                <button onClick={downloadHandler}>download</button>
              </Match>
              <Match when={selectedViewer() === "xml"}>
                <pre>{xml()}</pre>
              </Match>
              <Match when={selectedViewer() === "xsl"}>
                <pre>{xsl()}</pre>
              </Match>
              <Match when={selectedViewer() === "css"}>
                <pre>{css()}</pre>
              </Match>
            </Switch>
          )}
        </Show>
      </div>
    </>
  );
}
