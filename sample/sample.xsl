<xsl:stylesheet version="3.0" xmlns:xsl="http://www.w3.org/1999/XSL/Transform">

  <xsl:template match="/">
    <html>
      <head>
        <title>People List</title>
      </head>
      <body>
        <h1>People List</h1>
        <ul>
          <xsl:apply-templates select="people/person" />
        </ul>
      </body>
    </html>
  </xsl:template>

  <xsl:template match="person">
    <li>
      <xsl:value-of select="name" />, Age: <xsl:value-of select="age" />
    </li>
  </xsl:template>

</xsl:stylesheet>
