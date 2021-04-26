const fs = require('fs/promises');

const emojisList = async () => {
  return await fs.readFile('emoji-test.txt', 'utf-8');
};

function grouper(array, toFind) {
  return array.reduce((previous, current, index) => {
    if (current.includes(toFind)) {
      previous.push({ name: current, start: index });
    }
    return previous
  }, []);
}

const map = {};

(async function() {
  const list = await emojisList();
  let lines = list.split('\n');
  const indexes = grouper(lines, '# group:');

  indexes.forEach((group, index) => {
    map[`group${index}`] = {
      name: group.name.slice(9),
      start: group.start,
      subgroups: []
    }
  })

  for (let i = 0; i < indexes.length; i++) {
    let currentGroup = [];
    if (i + 1  < indexes.length) {
      currentGroup = lines.slice(indexes[i].start, indexes[i + 1].start);
    } else {
      currentGroup = lines.slice(indexes[i].start);
    }
    map[`group${i}`].subgroups = [...grouper(currentGroup, '# subgroup:')];
  }

  Object.entries(map).forEach(([key, value]) => {
    console.log('<>>>>>>>>>>>>>>>', key);
    const subgroups = value.subgroups;
    const groupStart = value.start;
    for(let j = 0; j < subgroups.length; j++) {
      let emojis = null;
      if (j + 1 < subgroups.length) {
        emojis = lines.slice(groupStart + subgroups[j].start + 1, groupStart + subgroups[j + 1].start - 1);
      } else {
        emojis = lines.slice(groupStart + subgroups[j].start + 1);
      }
      emojis = emojis.map((emoji) => {
        const a = emoji.replace(';fully-qualified', '');
        const emojiName = a.slice(a.search(/[0-9]\s[a-z]/) + 2).replace(/ +/g, '_').replace(/-/g, '_')
        const emojiCode = a.replace(/ +/g, '').slice(0, a.replace(/ +/g, '').indexOf(';'))
        return {
          emojiName,
          emojiCode
        }
      });

      map[key].emojis = emojis;
      delete map[key].start;
      delete map[key].subgroups;
    }
  });

  console.log(map)
})();
