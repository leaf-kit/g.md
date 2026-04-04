<p align="center">
  <img src="images/logo.png" alt="gmd logo" width="240" />
</p>

# gmd — **G**rep **M**ark**d**own

[![Release](https://img.shields.io/github/v/release/leaf-kit/g.md?label=release)](https://github.com/leaf-kit/g.md/releases/latest)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![Homebrew](https://img.shields.io/badge/homebrew-leaf--kit%2Fgmd-yellow.svg)](https://github.com/leaf-kit/homebrew-gmd)
[![Experimental](https://img.shields.io/badge/status-experimental-orange.svg)]()
[![pipe](https://img.shields.io/badge/pipe-grep%20%7C%20awk%20%7C%20jq-blue.svg)]()
[![AI Agent](https://img.shields.io/badge/agent--scan-claude%20%7C%20codex%20%7C%20copilot%20%7C%20cursor-purple.svg)]()

터미널에서 사용하는 빠르고 구조 인식이 가능한 마크다운 검색 및 분석 도구.

**gmd**는 마크다운 문서를 정밀하게 검색합니다 — 제목, 코드 블록, 링크, 굵은 텍스트, 인용문별로 필터링이 가능합니다. 할 일 추적, 이미지와 링크 유효성 검사, 태그 및 프론트매터 분석, AI 프롬프트용 컨텍스트 생성까지, 하나의 CLI로 모두 처리합니다.

대규모 마크다운 지식 베이스를 관리하는 **프롬프트 엔지니어**, 문서를 유지보수하는 **개발자**, 수백 개의 파일에서 노트를 정리하는 **작가**를 위한 터미널 필수 도구입니다.

## 왜 "gmd"인가?

**gmd**는 **Grep Markdown**의 약자입니다. `grep`이 텍스트를 검색하듯, **gmd**는 마크다운을 검색합니다 — 하지만 구조를 이해합니다. 제목과 코드 블록, 링크와 태그의 차이를 알고 있습니다. 패턴이 아닌, 의미를 검색하세요.

**gmd**는 **leaf**와 **bark**의 친구입니다. bark가 터미널에서 마크다운을 아름답게 렌더링하는 동안, gmd는 콘텐츠 속을 파고들어 정확히 필요한 것을 찾아냅니다.

> *텍스트만 검색하지 마세요. 의미를 검색하세요.*

## 기능

- **마크다운 구조 인식 검색** — 제목, 코드 블록, 링크, 굵은 텍스트, 인용문별 필터
- **할 일 관리** — 모든 문서의 `- [ ]` / `- [x]` 체크박스 추적
- **에셋 유효성 검사** — 깨진 이미지와 미사용 파일 검출
- **링크 점검** — 404 URL 탐지 및 백링크 검색
- **태그 분석** — 문서 전체의 `#해시태그` 목록, 순위, 검색
- **프론트매터 검사** — YAML 메타데이터 완전성 감사
- **AI 프롬프트 생성** — 전후 맥락이 포함된 AI 입력용 텍스트 추출
- **AI 에이전트 스캐너** — Claude, Codex, Copilot, Cursor, Windsurf, Aider, Cline 등 설정 탐지
- **내보내기** — 마크다운 또는 JSON 형식으로 연동용 출력
- **통계** — 단어 수, 읽기 시간, 활동 패턴
- 빠른 시작 — Rust로 작성, LTO 최적화

## 설치

### Homebrew (macOS)

```bash
brew tap leaf-kit/gmd
brew install gmd
```

### 소스에서 빌드

```bash
git clone https://github.com/leaf-kit/g.md.git
cd g.md
cargo build --release
cp target/release/gmd /usr/local/bin/
```

또는 빌드 스크립트 사용:

```bash
./build.sh
```

## 업데이트

### Homebrew

```bash
brew upgrade gmd
```

### 소스에서

```bash
git pull origin main
cargo build --release
cp target/release/gmd /usr/local/bin/
```

## 삭제

### Homebrew

```bash
brew uninstall gmd
brew untap leaf-kit/gmd
```

### 수동 (소스 설치)

```bash
rm /usr/local/bin/gmd
```

## 사용법

```
% gmd
Markdown 문서를 검색, 분석, 관리하는 빠른 CLI 도구.
Rust로 만들어 속도와 안전성을 확보했습니다.

Usage: gmd [OPTIONS] <COMMAND>

Commands:
  todo    마크다운 체크박스 및 할 일 관리
  find    정밀 필터로 마크다운 콘텐츠 검색
  img     문서 내 이미지 점검 및 관리
  link    링크 분석 및 유효성 검사
  tag     문서 전체 해시태그 관리
  meta    프론트매터 메타데이터 검사
  export  검색 결과를 다양한 형식으로 내보내기
  prompt  AI용 프롬프트 컨텍스트 생성
  path    파일 경로만 출력 (내용 없이)
  stat    문서 통계 표시
  claude  Claude Code 에이전트 설정 스캔
  codex   OpenAI Codex 에이전트 설정 스캔
  agents  모든 AI 에이전트 설정 스캔
  ui      인터랙티브 TUI 탐색 모드
  clean   중복 및 빈 파일 정리 제안
  help    도움말 출력

Options:
  -p, --path <PATH>  작업 디렉터리 설정 (기본값: 현재 디렉터리)
  -v, --verbose      상세 출력 활성화
  -q, --quiet        부수적인 출력 억제
  -h, --help         도움말 출력
  -V, --version      버전 출력
```

## 명령어 및 실행 결과 예시

아래 모든 예시는 포함된 `tree/` 샘플 문서(10개의 마크다운 파일: `docs/`, `notes/`, `projects/`)에서 실제 실행한 결과입니다.

### 1. 할 일 관리 (`todo`)

모든 문서에서 마크다운 체크박스(`- [ ]` / `- [x]`)를 검색하고 필터링합니다.

```
% gmd todo
  projects/sprint-backlog.md:18 - [ ] Migrate user service to Rust
  projects/sprint-backlog.md:19 - [ ] Write integration tests for API gateway
  projects/sprint-backlog.md:20 - [ ] Set up **Prometheus** monitoring
  docs/deployment.md:51 - [ ] Update Kubernetes manifests for v2.2
  notes/meeting-2026-03.md:20 - [ ] Alice: Draft migration plan by March 12
  notes/ideas.md:10 - [ ] Fuzzy search with Levenshtein distance
  ...

37 task(s) found.
```

```
% gmd todo done
  projects/sprint-backlog.md:22 - [x] Create database migration scripts
  projects/tech-debt.md:18 - [x] Migrate from `moment.js` to `chrono`
  docs/deployment.md:54 - [x] Write deployment documentation
  notes/ideas.md:13 - [x] Case-insensitive search
  ...

19 task(s) found.
```

```
% gmd todo migration
  projects/sprint-backlog.md:36 - [ ] Write blog post about migration
  notes/meeting-2026-03.md:20 - [ ] Alice: Draft migration plan by March 12

2 task(s) found.
```

| 명령어 | 설명 |
|--------|------|
| `gmd todo` | 모든 미완료 할 일 목록 |
| `gmd todo done` | 완료된 할 일만 보기 |
| `gmd todo today` | 오늘 수정된 파일의 할 일 |
| `gmd todo week` | 이번 주 수정된 파일의 할 일 |
| `gmd todo <QUERY>` | 키워드로 할 일 검색 |

### 2. 정밀 검색 (`find`)

구조 인식 필터로 문서 콘텐츠를 검색합니다.

```
% gmd find Rust
  projects/sprint-backlog.md:10 - Complete **Rust migration** for core services
  docs/architecture.md:22 | Backend | Rust (Actix-web) |
  notes/meeting-2026-03.md:14 - Migrate to **Rust** backend by end of Q2
  notes/meeting-2026-03.md:46 > "Rust gives us the safety guarantees we need for production." — Bob
  ...

13 match(es) found.
```

```
% gmd find head API
  docs/api-guide.md:7 # API Guide

1 match(es) found.
```

```
% gmd find code query
  notes/daily-log.md:12 [rust] fn search_code_blocks(content: &str, query: &str) -> Vec<Match> {
  notes/daily-log.md:15 [rust]         .filter(|b| b.content.contains(query))

2 match(es) found.
```

| 명령어 | 설명 |
|--------|------|
| `gmd find <QUERY>` | 전체 텍스트 검색 |
| `gmd find head <QUERY>` | 제목(`# Heading`) 내 검색 |
| `gmd find code <QUERY>` | 코드 블록 내부만 검색 |
| `gmd find link <QUERY>` | 링크 URL 및 위키링크 검색 |
| `gmd find bold <QUERY>` | 굵은 텍스트(`**bold**`) 검색 |
| `gmd find quote <QUERY>` | 인용문(`> quote`) 검색 |

### 3. 에셋 및 링크 점검 (`img`, `link`)

이미지와 링크의 무결성을 검증합니다.

```
% gmd img broken
  projects/sprint-backlog.md:51 BROKEN ![Burndown Chart](../images/burndown.png)
  docs/architecture.md:50 BROKEN ![System Diagram](../images/architecture.png)
  README.md:28 BROKEN ![Missing Banner](images/banner.png)

4 broken image(s) found.
```

```
% gmd img orphan
  ORPHAN images/orphan-unused.png

1 orphan image(s) found.
```

```
% gmd link broken
  README.md:17 BROKEN [Broken Link Example](docs/nonexistent.md)
  README.md:23 ERROR [Broken External](https://example.invalid/broken-page)
  notes/ideas.md:38 BROKEN [[Daily Log]]
  ...

7 broken link(s) found.
```

```
% gmd link back architecture
  projects/sprint-backlog.md:31 links to: ../docs/architecture.md
  docs/api-guide.md:73 links to: [[Architecture Overview]]
  README.md:14 links to: docs/architecture.md
  notes/meeting-2026-03.md:56 links to: ../docs/architecture.md

4 backlink(s) found.
```

| 명령어 | 설명 |
|--------|------|
| `gmd img list` | 문서 내 모든 이미지 목록 |
| `gmd img broken` | 깨진 이미지 참조 검출 |
| `gmd img orphan` | 미사용 이미지 파일 검출 |
| `gmd link list` | 모든 내부/외부 링크 추출 |
| `gmd link broken` | 깨진 URL(404) 탐지 |
| `gmd link back <FILE>` | 특정 문서의 백링크 검색 |

### 4. 태그 및 메타데이터 (`tag`, `meta`)

해시태그와 프론트매터 메타데이터를 관리합니다.

```
% gmd tag list
  #api                            (1)
  #architecture                   (1)
  #rust                           (2)
  #sprint                         (1)
  ...

28 unique tag(s) found.
```

```
% gmd tag top
  Rank  Tag                            Count
  ---------------------------------------------
  1     #rust                          2
  2     #security                      1
  3     #architecture                  1
  ...
```

```
% gmd meta missing
  PARTIAL docs/deployment.md (empty: author)
  NO META CLAUDE.md

2 document(s) with missing/empty metadata.
```

| 명령어 | 설명 |
|--------|------|
| `gmd tag list` | 모든 태그와 빈도 수 |
| `gmd tag top` | 가장 많이 사용된 태그 순위 |
| `gmd tag <NAME>` | 특정 태그가 포함된 파일 검색 |
| `gmd meta list` | 프론트매터 필드 목록 |
| `gmd meta missing` | 메타데이터 누락 문서 검출 |

### 5. 내보내기 및 AI 프롬프트 (`export`, `prompt`)

데이터를 내보내고 AI용 컨텍스트를 생성합니다.

```
% gmd prompt monitoring
You are analyzing a collection of Markdown documents.
Search query: "monitoring"

---

## Relevant Context

### File: projects/sprint-backlog.md

    L18: - [ ] Migrate user service to Rust
>>> L20: - [ ] Set up **Prometheus** monitoring

### File: notes/meeting-2026-03.md

>>> L16: - Budget approved for new **monitoring** tools
>>> L22: - [ ] Charlie: Research monitoring solutions (Datadog vs Grafana)

---

Based on the above context from the Markdown documents, please provide your analysis.
```

| 명령어 | 설명 |
|--------|------|
| `gmd export md` | 결과를 하나의 마크다운 파일로 합치기 |
| `gmd export json` | JSON으로 내보내기 |
| `gmd prompt <QUERY>` | AI 프롬프트용 컨텍스트 생성 |
| `gmd path` | 파일 경로만 출력 |

### 6. 시스템 및 통계 (`stat`, `clean`)

전체 현황 파악 및 유지보수.

```
% gmd stat
  GMD Statistics
  ========================================
  Documents:                10
  Total size:               10.2 KB
  Total lines:              470
  Total words:              1,532
  Total characters:         10,427
  ----------------------------------------
  Avg words/doc:            153
  Avg lines/doc:            47
```

```
% gmd stat time
  Reading Time & Activity
  ==================================================
  Total reading time:                 7 min (1,532 words)

  File                                        Words     Time
  ------------------------------------------------------------
  notes/meeting-2026-03.md                      211     1 min
  projects/sprint-backlog.md                    208     1 min
  docs/api-guide.md                             187     0 min
  ...
```

| 명령어 | 설명 |
|--------|------|
| `gmd stat` | 문서 수, 용량, 단어 수 통계 |
| `gmd stat time` | 읽기 시간 및 활동 패턴 분석 |
| `gmd clean` | 빈 파일/중복/짧은 파일 탐지 |

### 7. AI 에이전트 스캐너 (`claude`, `codex`, `agents`)

AI 코딩 에이전트 설정을 트리 형태로 탐지하여 상태 표시와 함께 출력합니다.

```
% gmd claude
  AI Coding Agent Configuration Scanner
  ═══════════════════════════════════════════════════════

  Claude Code [READY]
  Anthropic Claude Code - AI coding assistant

    ├──  ✓ CLAUDE.md (205 B)
    ├──  ○ .claude/ [optional]
    ├──  ○ .claude/settings.json [optional]
    ├──  ○ .claude/commands/ [optional]
    └──  ○ .claude/commands.md [optional]
```

| 명령어 | 설명 |
|--------|------|
| `gmd claude` | Claude Code 설정 스캔 |
| `gmd codex` | OpenAI Codex 설정 스캔 |
| `gmd agents` | 모든 AI 에이전트 설정 스캔 |

지원 에이전트: Claude Code, OpenAI Codex, GitHub Copilot, Cursor, Windsurf (Codeium), Aider, Continue.dev, Cline, Amazon Q Developer, Gemini Code Assist.

## 파이프 연동 (`|`)

gmd는 표준 출력(stdout)으로 텍스트를 출력하므로, Unix 표준 도구들과 파이프(`|`)로 자연스럽게 연결됩니다. 아래는 테스트된 실용적인 시나리오입니다.

### 유용한 파이프 조합

**`grep`으로 검색 결과 재필터** — "Rust" 검색 결과 중 회의록만:

```
% gmd find Rust | grep "meeting"
  notes/meeting-2026-03.md:14 - Migrate to **Rust** backend by end of Q2
  notes/meeting-2026-03.md:37 Projected (Rust):
  notes/meeting-2026-03.md:46 > "Rust gives us the safety guarantees we need for production." — Bob
```

**미완료 할 일 개수 세기** — `grep -c`로 빠른 집계:

```
% gmd todo | grep -c "\- \[ \]"
37
```

**파일별 할 일 집계** — 어떤 파일에 할 일이 가장 많은지:

```
% gmd todo | grep "\- \[ \]" | awk -F: '{print $1}' | sort | uniq -c | sort -rn
  10   projects/sprint-backlog.md
   9   notes/ideas.md
   8   projects/tech-debt.md
   5   notes/meeting-2026-03.md
   3   docs/deployment.md
   2   notes/daily-log.md
```

**파일별 매치 수 집계** — 키워드가 어디에 집중되어 있는지:

```
% gmd find Rust | grep -v "match" | awk -F: '{print $1}' | sort | uniq -c | sort -rn
   4   projects/sprint-backlog.md
   4   notes/meeting-2026-03.md
   2   CLAUDE.md
   1   README.md
```

**문서별 라인 수** — `gmd path`와 `xargs wc` 조합:

```
% gmd path 2>/dev/null | sed 's|^|./|' | xargs wc -l
      54 ./projects/sprint-backlog.md
      75 ./docs/api-guide.md
      58 ./notes/meeting-2026-03.md
     508 total
```

**깨진 이미지 경로만 추출** — 스크립트용 깔끔한 리스트:

```
% gmd img broken 2>&1 | grep "BROKEN" | sed 's/.*!\[.*\](//;s/).*//'
../images/burndown.png
../images/velocity.png
images/banner.png
```

**AI 프롬프트를 클립보드에 복사** (macOS):

```
% gmd prompt "authentication" | pbcopy
```

**JSON 내보내기를 `jq`로 필터**:

```
% gmd export json | jq '.[].file'
"projects/sprint-backlog.md"
"projects/tech-debt.md"
"docs/architecture.md"
...
```

### 파이프 사용 시 유의사항

> **gmd를 파이프와 함께 사용할 때 중요한 주의사항:**

1. **파이프에서는 컬러 출력이 자동 비활성화됩니다.** gmd는 터미널에서 ANSI 색상을 사용하지만, `colored` 라이브러리가 비-TTY 출력을 감지하여 자동으로 비활성화합니다. 파이프 출력은 깨끗한 텍스트입니다.

2. **요약 라인은 stdout에 포함됩니다.** `37 task(s) found.` 같은 라인이 stdout에 포함됩니다. `grep -v "found\|match"`로 필터링하거나 `2>/dev/null`로 stderr 요약을 억제하세요.

3. **`gmd path`는 stdout과 stderr 모두 사용합니다.** 파일 경로는 stdout, 카운트 요약(`10 file(s).`)은 stderr입니다. `xargs` 연결 시 `2>/dev/null`을 사용하세요.

4. **파이프 사용 시 `-p` 플래그를 먼저 지정하세요.** 다른 도구와 조합할 때 `-p <PATH>`로 작업 디렉터리를 명시하면, 출력의 상대 경로가 예측 가능합니다.

5. **`gmd link broken`은 외부 URL로 인해 느려질 수 있습니다.** 각 외부 링크마다 HTTP HEAD 요청(10초 타임아웃)을 보냅니다. 대형 프로젝트에서는 `grep -v "EXT"`로 먼저 필터링하세요.

6. **인코딩: gmd는 UTF-8을 출력합니다.** ASCII만 기대하는 도구와 파이프할 때 한글 태그 등에서 문제가 생길 수 있습니다. 필요 시 `LC_ALL=en_US.UTF-8`을 설정하세요.

## 문서

- [English README](README.md) — 영문 문서
- [한국어 매뉴얼](README_ko.md) — 이 문서

## 피드백 및 기여

이 프로젝트는 초기 릴리즈이며 아직 개선의 여지가 있습니다. 매 업데이트마다 gmd를 더 좋게 만들기 위해 노력하고 있습니다. 문제가 있거나 제안사항이 있으면 이슈를 열어주세요:

[github.com/leaf-kit/g.md/issues](https://github.com/leaf-kit/g.md/issues)

여러분의 피드백이 개선에 도움이 됩니다. 건강하고 행복하세요!

## 라이선스

[MIT](LICENSE)
