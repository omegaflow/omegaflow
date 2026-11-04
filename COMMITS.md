# Commit messages

A commit message is the only place that carries the *why*. The code carries the
what. A reader who was not in the session decides from the message whether the
change is right.

## Form

```
area: the change in one sentence

Why the change is needed, in whole sentences. What was measured, with the
numbers and the place. What was rejected, and why. What stays open, named as
pending.

Checked with cargo check --workspace under -D warnings.
```

- `area` is one lowercase word, the part of the tree the change touches:
  `pages`, `tdm`, `sources`, `model`, `ci`, `tools/register`.
- The subject is at most 72 characters, starts lowercase after the colon and
  carries no closing period.
- One blank line follows the subject.
- The body explains the why. A change to code carries a body.
- Body lines are at most 100 characters.
- The message is English. German belongs to the documents, not to the commits.
- A number carries its unit and where it was measured.
- A claim without a measurement is written as `pending`, never as a result.

## Where this form comes from

The form is binding in the workshop repository (`bindend/commit.md`); this file
is its public restatement, so a reader of this repository sees the same rule.

The measured part rests on Tian, Zhang, Stol, Jiang and Liu, *What makes a good
commit message?*, ICSE 2022, DOI 10.1145/3510003.3510205: across 1,649 labelled
messages from five active projects, a good message carries the why and the what,
and about 44 percent of messages could be improved. The 50/72 line width is a
convention from Tim Pope and Chris Beams, not a measurement; it is used here for
reading comfort and named as such.

## What the gate refuses

`.github/workflows/commit-message.yml` checks every pushed commit and fails the
run when one of these holds:

1. the subject does not match `^[a-z0-9_.-]+: [a-z0-9]`
2. the subject is longer than 72 characters or ends with a period
3. the subject is followed by text without a blank line
4. a code commit carries no body
5. a line is longer than 100 characters
6. the message carries an umlaut (`ä ö ü ß`) or a German stopword
7. the message carries a home path, a personal handle, a phone number or a
   private directory name

Merge commits are exempt from rule 3.

The gate is the wall. Prose in a template is not.
