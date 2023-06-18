// Dummy file for AST

interface Param<ParamType> {
  type: ParamType,
  value: String,
}

export default [
  {
    name: "main",
    action: [
      {
        type: "Strandard",
        name: "print",
        params: ["Param<String>"],
      },
      {
        type: "Custom",
        name: "my_func",
        params: [],
      },
    ],
  },
];
