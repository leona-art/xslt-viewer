import { component$, useSignal,$ ,useVisibleTask$} from '@builder.io/qwik';
import type { Signal } from '@builder.io/qwik';
import {open as DialogOpen} from "@tauri-apps/api/dialog"
import { invoke } from '@tauri-apps/api';

export default component$(() => {
  const xml=useSignal("");
  const xsl=useSignal("");
  const result=useSignal("");
  useVisibleTask$(async({track})=>{
    track(()=>xml.value)
    track(()=>xsl.value)
    if(xml.value&&xsl.value){
      result.value=xslt(xml.value,xsl.value);
    }
  })
  return (
    <div class="pt-2 px-2">
      <div class="flex flex-row justify-evenly">
        <Dialog title="XML" ext='xml' content={xml} />
        <Dialog title="XSL" ext='xsl' content={xsl}/>
      </div>

      <h1>Result</h1>
      <div dangerouslySetInnerHTML={result.value}></div>
      <pre>{result}</pre>
    </div>
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
    const result = await DialogOpen({filters:[{name:title,extensions:[ext??"*"]}]});
    if(result&&!Array.isArray(result)){
      path.value=result;
    }
  })
  useVisibleTask$(async({track})=>{
    track(()=>path.value)
    if(content){
      content.value= await invoke("read_file",{path:path.value}).catch(()=>"") as unknown as string;
    }
    
  })
  return (
    <div class="block">
      <h1 class="text-10">{title}</h1>
      <input type="text" bind:value={path} class="border border-slate-300"/>
      <button class="bg-sky-500 text-white px-1 mx-1 rounded-md" onClick$={select}>Select</button>
    </div>
  );
})

function xslt(xml:string,xslt:string){
    const xsltProcessor = new XSLTProcessor();
    const parser = new DOMParser();
    const xmlDom = parser.parseFromString(xml, 'text/xml');
    const xsltDom = parser.parseFromString(xslt, 'text/xml');
    xsltProcessor.importStylesheet(xsltDom);
    const resultDocument = xsltProcessor.transformToDocument(xmlDom);
    return new XMLSerializer().serializeToString(resultDocument);
}