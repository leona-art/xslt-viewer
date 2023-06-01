import { component$, useSignal,$ } from '@builder.io/qwik';
import type { Signal } from '@builder.io/qwik';
import {open as DialogOpen} from "@tauri-apps/api/dialog"

export default component$(() => {
  const xml=useSignal("");
  return (
    <>
      <Dialog title="XML" ext='xml' content={xml} />
    </>
  );
});

type DialogProps = {
  title: string;
  ext?:string;
  content?:Signal<string>;
}
const Dialog = component$<DialogProps>(({title,content,ext}) => {
  const path=useSignal("")
  const select=$(async()=>{
    const result = await DialogOpen({filters:[{name:title,extensions:[ext??"*"]}]}).catch(()=>"");
    if(result&&!Array.isArray(result)){
      path.value=result;
      
    }
  })
  return (
    <>
    <h1>{title}</h1>
    <input type="text" bind:value={path}/>
    <button onClick$={select}>Select</button>
    </>
  );
})

function xslt(xml:string,xslt:string){
    const xsltProcessor = new XSLTProcessor();
    const parser = new DOMParser();
    const xmlDom = parser.parseFromString(xml, 'text/xml');
    const xsltDom = parser.parseFromString(xslt, 'text/xml');
    xsltProcessor.importStylesheet(xsltDom);
    const resultDocument = xsltProcessor.transformToDocument(xmlDom);
    return new XMLSerializer().serializeToString(resultDocument)
}