"use strict";
delete Object.prototype.__proto__;

((window) => {
  const core = window.__bootstrap.core;
  const { ObjectAssign } = window.__bootstrap.primordials;

  const byond = {
    async href(data) {
      return await core.opAsync("op_byond_href", data);
    },
  };

  ObjectAssign(globalThis, { byond });
})(globalThis);
