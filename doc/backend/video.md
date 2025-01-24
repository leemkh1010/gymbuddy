```mermaid
flowchart TD
  rj[Receive job] --> dv[Download video] --> p[Pre-process video]
  p --> ef[Extract frames] --> kp[Key points] --> ckip[Calculate key interest points]
  ckip --> dkp[Draw key points] --> uv[Upload video] --> Complete

```