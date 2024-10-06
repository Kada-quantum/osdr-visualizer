function generateFlowchart(arr) {
  if (arr.length !== 0) {
    var DST = "st=>start: " + arr[0] + "\n";
    if (arr.length > 1) {
      DST += "e=>end: " + arr[arr.length-1] + "\n";
      }
    if (arr.length > 2) {
        var ids = [];
        for (var i = 1; i < arr.length-1; i++) {
          let id = "o" + i;
          DST += id + "=>operation: " + arr[i] + "\n";
          ids.push(id);
        }
      }
    DST += "st->";
    ids.forEach(e=>{DST += e+"->"});
    DST += "e\n";
    console.log(DST);
    var diagram = flowchart.parse(DST);
    diagram.drawSVG('draw');
  }
}

function get_result() {
  var ul1 = document.getElementById('ul1');
  var result = [];
  var items = ul1.getElementsByTagName('li');
  for (var j = 0, m = items.length; j < m; j++) {
    var str = items[j].innerHTML;
    if (result.indexOf(str) == -1) {
      result.push(str);
    }
  }
  return result;
}

window.onload = (event) => {
  var result = get_result();
  generateFlowchart(result);
};
