// Web utilities

// x and y are relative to the viewport
export function caretPositionFromPoint(
  x: number,
  y: number,
): {textNode: Node, offset: number} | null {
  let textNode, offset;
  if ((<any>document).caretPositionFromPoint) {
    let range = (<any>document).caretPositionFromPoint(x, y);
    if (!range) {
      return null;
    }
    textNode = range.offsetNode;
    offset = range.offset;
  } else if (document.caretRangeFromPoint) {
    let range = (<any>document).caretRangeFromPoint(x, y);
    if (!range) {
      return null;
    }
    // console.log('HELP2', textNode, offset);
    textNode = range.startContainer;
    offset = range.startOffset;
  } else {
    return null;
  }

  if (textNode === null) {
    return null;
  }

  return {
    textNode,
    offset,
  };
}
  
export function textNodeAtPoint(
  x: number,
  y: number,
): {textNode: Text, offset: number} | null {
  let result = caretPositionFromPoint(x, y);

  // Filter out all non-element positions.
  if (result != null && result.textNode.nodeType !== 3) {
    return null;
  }

  return result as any;
}

export function matchesSelector(
  el: Node,
  selector: String,
): boolean {
  let matches = 
    (<any>Element.prototype).matchesSelector || 
    (<any>Element.prototype).mozMatchesSelector ||
    (<any>Element.prototype).msMatchesSelector || 
    (<any>Element.prototype).oMatchesSelector || 
    (<any>Element.prototype).webkitMatchesSelector ||
    function(s: any) {
        var matches = (this.document || this.ownerDocument).querySelectorAll(s),
            i = matches.length;
        while (--i >= 0 && matches.item(i) !== this) {}
        return i > -1;            
    };

  return matches.call(el, selector);
}
