function downloadTextFile(filename, textContent) {
    const blob = new Blob([textContent], {type: 'text/plain'});

    const url = URL.createObjectURL(blob);

    const link = document.createElement('a');

    link.href = url;
    link.download = filename;

    document.body.appendChild(link);

    link.click();

    document.body.removeChild(link);
    URL.revokeObjectURL(url);
}

var text = "const TAGS: [(&str, i32); 0] = [";
document.querySelectorAll('[data-class="filter-tooltip"]').forEach(e => text += "(\"" + e.textContent.replaceAll("\n", "") + "\", " + e.getAttribute("id") + "),")
downloadTextFile("test.json", text + "];")