export function xslt(xml: string, xsl: string) {
    if (xml === "") {
      return ""
    }
    if (xsl === "") {
      return xml
    }
    try {
      const parser = new DOMParser()
      const xmlDoc = parser.parseFromString(xml, "text/xml")
      const xslDoc = parser.parseFromString(xsl, "text/xml")
  
      const xsltProcessor = new XSLTProcessor()
      xsltProcessor.importStylesheet(xslDoc)
  
      const transformerDoc = xsltProcessor.transformToDocument(xmlDoc)
      return new XMLSerializer().serializeToString(transformerDoc)
    } catch (e) {
      console.log(e)
      return
    }
  }